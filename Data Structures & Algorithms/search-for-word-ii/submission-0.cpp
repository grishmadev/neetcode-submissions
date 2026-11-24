constexpr int A = 'a';
class Solution {
    struct TrieNode {
        TrieNode* children[26] = {nullptr};
        string word;
    };

private:
    void insertWord(const string &w, TrieNode* node) {
        TrieNode* cur = node;
        for (char ch : w) {
            int idx = ch - A;
            if (!cur->children[idx]) cur->children[idx] = new TrieNode();
            cur = cur->children[idx];
        }
        cur->word = w;
    }

    bool hasChild(TrieNode* node) {
        for (int i = 0; i < 26; ++i)
            if (node->children[i])
                return true;
        return false;
    }

    void dfs(vector<vector<char>> &board, int c, int r, TrieNode* parent, vector<string> &res) {
        int rows = static_cast<int>(board.size());
        int cols = static_cast<int>(board[0].size());

        if (c < 0 || r < 0 || r >= rows || c >= cols || board[r][c] == '#') return;

        char ch = board[r][c];
        int idx = ch - A;
        TrieNode* cur = parent->children[idx];

        if (!cur) return;

        if (!cur->word.empty()) {
            res.push_back(move(cur->word));
            cur->word.clear();
        }

        board[r][c] = '#';

        dfs(board, c - 1, r, cur, res);
        dfs(board, c, r + 1, cur, res);
        dfs(board, c + 1, r, cur, res);
        dfs(board, c, r - 1, cur, res);

        board[r][c] = ch;

        if (cur->word.empty() && !hasChild(cur)) {
            delete cur;
            parent->children[idx] = nullptr;
        }
    }
public:
    vector<string> findWords(vector<vector<char>>& board, vector<string>& words) {
        vector<string> res;
        if (board.empty() || board[0].empty() || words.empty()) return res;

        TrieNode *root = new TrieNode();
        for (const string &word : words) insertWord(word, root);
        
        for (int r = 0; r < static_cast<int>(board.size()); ++r) {
            for (int c = 0; c < static_cast<int>(board[0].size()); ++c) {
                dfs(board, c, r, root, res);
            }
        }

        delete root;
        return res;
    }
};
