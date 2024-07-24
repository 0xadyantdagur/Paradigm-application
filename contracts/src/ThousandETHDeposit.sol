// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

contract ThousandETHDeposit {
    // mapping of depositor addresses to amount deposited
    mapping(address => uint256) public depositorAmounts;

    address[] depositors;

    // total value deposited
    uint256 public ETH_DEPOSITED = 0;

    // creator address
    address internal immutable creator;

    constructor() payable {
        creator = msg.sender;
    }

    // can recieve native ETH
    function depositEth() external payable {
        require(msg.value >= 0, "insufficient msg.value, must be > 0");
        address _address = msg.sender;
        uint256 _amt = msg.value;

        ETH_DEPOSITED += _amt;
        depositorAmounts[_address] += _amt;

        if (!depositorsContains(_address)) {
            depositors.push(_address);
        }
    }

    // repays the depositors pro-rata
    function repayDepositors() external {
        require(
            msg.sender == creator,
            "only the contract creator must call this function"
        );

        require(address(this).balance >= 0, "insufficient contract balance");
        uint256 amount_won = address(this).balance;

        for (uint256 i = 0; i < depositors.length; i++) {
            address _depositor = depositors[i];
            uint256 depositAmount = depositorAmounts[_depositor];

            if (depositAmount > 0) {
                uint256 share = (depositAmount * amount_won) / ETH_DEPOSITED;
                payable(_depositor).transfer(share);
                depositorAmounts[_depositor] = 0;
            }
        }

        ETH_DEPOSITED = 0;
    }

    // lazy implementation of checking if an array contains a value
    function depositorsContains(address _address) internal view returns (bool) {
        for (uint256 i = 0; i < depositors.length; i++) {
            if (depositors[i] == _address) {
                return true;
            }
        }
        return false;
    }
}
