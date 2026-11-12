class LRUCache {
private:
    unordered_map<int, list<pair<int, int>>::iterator> cache_map;
    list<pair<int, int>> cache_list;
    int capacity;

public:
    LRUCache(int capacity) {
        this->capacity = capacity;
    }
    
    int get(int key) {
        // if empty
        if (cache_map.find(key) == cache_map.end()) {
            return -1;
        }
        cache_list.splice(cache_list.begin(), cache_list, cache_map[key]);
        return cache_map[key]->second;
    }
    
    void put(int key, int value) {
        if (cache_map.find(key) != cache_map.end()) {
            cache_map[key]->second = value;
            cache_list.splice(cache_list.begin(), cache_list, cache_map[key]);
            return;
        }

        if (cache_list.size() == capacity) {
            auto last = cache_list.back();
            cache_map.erase(last.first);
            cache_list.pop_back();
        }

        cache_list.push_front({key, value});
        cache_map[key] = cache_list.begin();
    }
};
