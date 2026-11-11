class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        unordered_set<int> set = {};
        for (int x : nums) {
            if (set.contains(x)) {
                return x;
            }
            set.insert(x);
        }
        return 0;
    }
};
