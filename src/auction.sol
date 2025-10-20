// SPDX-License-Identifier: MIT

pragma solidity ^0.8.13;

import "./nft.sol";
import "./sapicoin.sol";

contract Auction {
    ERC20 immutable public currency;
    
    struct Nft {
        uint id;
        ERC721 collection;
    }

    address immutable public CREATOR;
    bool public auctionEnded;
    Nft toSold;

    mapping (address => uint) bids;

    event Bid(address indexed creator, uint value);
    event EndedAuction(address indexed winner);


    /// @param token address to the smart contract of the token
    /// @param nft address to the smart constarct of the NFT
    /// @param tokenId id to be sold
    constructor(address token, address nft, uint tokenId) {
        CREATOR = msg.sender;
        currency = ERC20(token);
        toSold = Nft(tokenId, ERC721(nft));
        auctionEnded = false;
        require(toSold.collection.ownerOf(tokenId) == msg.sender, "you must be the owner of the NFT");
    }

    // track bidders with a set for O(1) membership + array for iteration
    address[] private bidders;
    mapping(address => bool) private isBidder;

    function winner() public view returns (address newOwner) {
        (newOwner, ) = topBid();
    }

    function endAuction() public returns (bool result) {
        // simple end: only creator can end and mark auction as ended
        require(!auctionEnded, "auction already ended");
        require(msg.sender == CREATOR, "you must be the creator if you want to terminate the auction");
        result = true;
        address newOwner = winner();
        toSold.collection.approve(newOwner, toSold.id);
        result = currency.transferFrom(newOwner, CREATOR, bids[newOwner]);
        require(result, "very wrong");
        emit EndedAuction(newOwner);
        auctionEnded = true; // now the contract is unusable for further endings
    }


    function placeBid(uint value) public returns (bool result) {
        // pre-conditions: the user have enough token
        require(currency.balanceOf(msg.sender) >= value, "not enough tokens");
        result = currency.approve(address(this), value); // approva una futura transazione da me verso il contratto
        require(result, "something went wrong when approving");
        if(!isBidder[msg.sender]) {
            isBidder[msg.sender] = true;
            bidders.push(msg.sender);
        }
        bids[msg.sender] = value;
        emit Bid(msg.sender, value);
    }

    function topBid() public view returns (address user, uint value) {
        value = 0;
        user = address(0);

        for (uint i = 0; i < bidders.length; i++) {
            address bidder = bidders[i];
            uint val = bids[bidder];
            if (val > value) {
                value = val;
                user = bidder;
            }
        }

    }
}