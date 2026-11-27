constexpr int A = 'A';
class Solution {
public:
    int leastInterval(vector<char>& tasks, int n) {
        int array[26] = {0};
        for (char task : tasks) array[task - A]++; 
        priority_queue<int> pq;
        for (int c : array) if (c) pq.push(c);
        int t = 0;
        queue<pair<int, int>> q;
        while (!pq.empty() || !q.empty()) {
            if (pq.empty() && !q.empty() && t < q.front().second) t = q.front().second;
            while (!q.empty() && q.front().second <= t) {
                pq.push(q.front().first);
                q.pop();
            }
            int rem = pq.top() - 1;
            pq.pop();
            if (rem > 0) q.push({rem, t + n + 1});
            t++;
        }
        return t;
    }
};
