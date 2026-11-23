constexpr int SIZE = 26;
constexpr int BASE_CHAR = 'a';

struct Trie {
  bool end;
  Trie *children[SIZE];

  Trie() : end(false) {
    for (int i = 0; i < SIZE; i++) {
      children[i] = nullptr;
    }
  }
  explicit Trie(bool end) : end(end) {
    for (int i = 0; i < SIZE; i++) {
      children[i] = nullptr;
    }
  }
};

class PrefixTree {
public:
  Trie *tree = new Trie();
  PrefixTree() {}
  void insert(string word) {
    Trie *thistree = tree;
    for (char ch : word) {
      int idx = int(ch) - BASE_CHAR;
      if (!thistree->children[idx]) {
        Trie *new_tree = new Trie();
        thistree->children[idx] = new_tree;
      }
      thistree = thistree->children[idx];
    }
    thistree->end = true;
  }

  bool search(string word) {
    Trie *node = this->find(word);
    return (node) ? node->end : false;
  }

  bool startsWith(string prefix) {
    return !!this->find(prefix);
  }

  Trie* find(const string &word) {
    Trie *node = tree;
    for (char ch : word) {
      int idx = ch - BASE_CHAR;
      if (!node->children[idx])
        return nullptr;
      node = node->children[idx];
    }
    return node;
  }
};
