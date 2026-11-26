class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        make_heap(stones.begin(), stones.end(), greater<>{});
        while (stones.size() > 1) {
            pop_heap(stones.begin(), stones.end());
            int stone1 = stones.back();
            stones.pop_back();
            pop_heap(stones.begin(), stones.end());
            int stone2 = stones.back();
            stones.pop_back();
            int diff = abs(stone2 - stone1);
            if (diff) stones.push_back(diff);
        }
        return stones.empty() ? 0 : stones[0];
    }
};
