constexpr int ALPHABETS = 26;
constexpr int BASE_CHAR = 'a';
struct Dic {
  bool end;
  Dic *children[26];
  Dic() : end(false) {
    for (int x = 0; x < ALPHABETS; x++) {
      children[x] = nullptr;
    }
  }
};

class WordDictionary {
  Dic *dict = nullptr;

public:
  WordDictionary() { this->dict = new Dic(); }

  void addWord(string word) {
    Dic *node = this->dict;
    for (int ch : word) {
      int idx = ch - BASE_CHAR;
      Dic *new_dict = new Dic();
      if (!node->children[idx])
        node->children[idx] = new_dict;
      node = node->children[idx];
    }
    node->end = true;
  }

  bool search(string word) {
    return present(word, 0, this->dict);
  }

  bool present(const string &word, int j, Dic *root) {
    if (!root) return false;
    if (j == word.size()) return root->end;
    char ch = word[j];
    if (ch == '.') {
        for (int k = 0; k < ALPHABETS; k++)
            if (root->children[k]
            && this->present(word, j + 1, root->children[k]))
            return true;
        return false;
    }
    int idx = ch - BASE_CHAR;
    if (root->children[idx] == nullptr)
        return false;
    return present(word, j + 1, root->children[idx]);
  }
};
