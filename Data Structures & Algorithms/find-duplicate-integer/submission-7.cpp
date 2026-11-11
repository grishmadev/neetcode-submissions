class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        int tortoise = 0, hare = 0;
        while (true) {
            tortoise = nums[tortoise];
            hare = nums[nums[hare]];
            if (hare == tortoise) break;
        }
        int tortoise_cousin = 0;
        while (true) {
            tortoise = nums[tortoise];
            tortoise_cousin = nums[tortoise_cousin];
            if (tortoise == tortoise_cousin) return tortoise;
        }
    }
};
