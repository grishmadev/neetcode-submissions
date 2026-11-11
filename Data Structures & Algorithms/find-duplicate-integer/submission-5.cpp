class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        uint size = nums.size();
        for(int i = 0; i < size; i++) {
            int idx = abs(nums[i]) - 1;
            if (nums[idx] < 0) {
                return -nums[idx];
            }
            nums[idx] *= -1;
        }
        return 0;
    }
};
