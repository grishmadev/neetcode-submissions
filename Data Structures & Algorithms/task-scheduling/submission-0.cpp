constexpr int A = 'A';
class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        char array[26] = {0};
        for (char task : tasks) array[task - A]++; 
        priority_queue<int, vector<int>> pq;
        for (int c : array) if (c) pq.push(c);
        int t = 0;
        queue<pair<int, int>> q;
        while (!pq.empty() || !q.empty()) {
            if (!pq.empty()) {
                int maxVal = pq.top() - 1;
                pq.pop();
                if (maxVal) q.push({ maxVal, t + n });
            }
            auto top = q.front();
            if (!q.empty() && top.second == t) {
                pq.push(top.first);
                q.pop();
            }
            t++;
        }
        return t;
    }
};
