class Solution {
public:
    int findDuplicate(vector<int>& nums) {
        int arr[16384] = {0};
        for (int x : nums) {
            if (arr[x]) {
                return x;
            }
            arr[x] = 1;
        }
        return 0;
    }
};
