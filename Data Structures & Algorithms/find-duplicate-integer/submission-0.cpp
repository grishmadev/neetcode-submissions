class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        int prev = 0;
        for (int x : nums) {
            if (x == prev) {
                return x;
            }
            prev = x;
        }
        return prev;
    }
};
