class Twitter {
    /// Current Time
    int time;
    /// Map(userId, Vec({ tweetId, timePosted }))
    unordered_map<int, vector<pair<int, int>>> tweets;
    /// Map(userId, Set(Followees));
    unordered_map<int, unordered_set<int>> followMap;
public:
    Twitter() {
        this->time = 0;
    }
    
    void postTweet(int userId, int tweetId) {
        auto& feed = this->tweets[userId];
        feed.push_back({ tweetId, this->time++ });
    }

    vector<int> getNewsFeed(int userId) {
        auto comp = [](const pair<int, int>& a, const pair<int, int>& b) {
            return a.second < b.second;
        };
        priority_queue<pair<int, int>, vector<pair<int, int>>, decltype(comp)> pq(comp);
        auto followees = this->followMap[userId];
        auto userFeed = this->tweets[userId];
        if (auto it = this->tweets.find(userId); it != this->tweets.end())
            for (const auto& tweet : it->second) pq.push(tweet);
        if (auto fit = this->followMap.find(userId); fit != this->followMap.end())
            for (int fId : fit->second)
                if (auto tit = this->tweets.find(fId); tit != this->tweets.end())
                    for (const auto& tweet : tit->second) pq.push(tweet);
        vector<int> res;
        int count = 0;
        while (!pq.empty() && count < 10) {
            res.push_back(pq.top().first);
            pq.pop();
            count++;
        }
        return res;
    }
    
    void follow(int followerId, int followeeId) {
        if (followerId == followeeId) return;
        this->followMap[followerId].insert(followeeId);
    }
    
    void unfollow(int followerId, int followeeId) {    
        if (followerId == followeeId) return;
        if (auto it = this->followMap.find(followerId); it != this->followMap.end())
            it->second.erase(followeeId);
    }
};
