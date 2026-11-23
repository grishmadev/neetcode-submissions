constexpr int ALPHABETS = 26;
constexpr int BASE = 'a';

class PrefixTree {
private:
    bool end;
    PrefixTree *children[26];
public:
    PrefixTree() {
        this->end = false;
        for (int i = 0; i < ALPHABETS; i++) {
            this->children[i] = nullptr;
        }
    }
    
    void insert(string word) {
        PrefixTree* node = this;
        for (char ch : word) {
            int idx = ch - BASE;
            PrefixTree* new_node = new PrefixTree();
            if (!node->children[idx]) node->children[idx] = new_node;
            node = node->children[idx];
        }
        node->end = true;
    }
    
    bool search(string word) {
        PrefixTree* node = this->find(word);
        return node ? node->end : false;
    }
    
    bool startsWith(string prefix) {
        return !!this->find(prefix);
    }

    PrefixTree* find(const string &word) {
        PrefixTree* node = this;
        for (char ch : word) {
            int idx = ch - BASE;
            if (!node->children[idx]) return nullptr;
            node = node->children[idx];
        }
        return node;
    }
};
