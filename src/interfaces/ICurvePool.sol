// SPDX-License-Identifier: MIT
pragma solidity 0.8.21;

interface ICurvePool {
    function exchange(int128 i, int128 j, uint256 dx, uint256 min_dy) external payable returns (uint256);
    function coins(int128 i) external view returns (address);
}
