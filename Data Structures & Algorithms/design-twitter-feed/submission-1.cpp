class Twitter {
unordered_map<int, pair<unordered_set<int>,vector<int>>> map;        
public:
    Twitter() {
        // map of userId, pair (vector of followees, vector of postIds)
        // unordered_map<int, pair<vector<int>,vector<int>>> map;        
    }
    
    void postTweet(int userId, int tweetId) {
        auto present = this->map.contains(userId);
        if (!present) this->map.insert({ userId, {{}, { tweetId }}});
        else this->map[userId].second.push_back(tweetId);
    }
    
    vector<int> getNewsFeed(int userId) {
        if (!this->map.contains(userId)) return {};
        auto user = this->map[userId];
        priority_queue<int> pq;
        for (const int& following : user.first) {
            auto user = this->map[following];
            for (int post : user.second) pq.push(post);
        }
        for (int post : user.second) pq.push(post);
        vector<int> res;
        while (!pq.empty() && res.size() < 10) {
            res.push_back(pq.top());
            pq.pop();
        }
        return res;
    }
    
    void follow(int followerId, int followeeId) {
        if (followerId == followeeId) return;
        this->map[followerId].first.insert(followeeId);
    }
    
    void unfollow(int followerId, int followeeId) {
        this->map[followerId].first.erase(followeeId);
    }
};
