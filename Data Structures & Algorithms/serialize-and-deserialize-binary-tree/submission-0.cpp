/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
#include <cstddef>
class Codec {
public:

    // Encodes a tree to a single string.
    string serialize(TreeNode* root) {
        vector<string> nodes;
        ser_dfs(root, nodes);
        return join(nodes, "-");
    }

    void ser_dfs(TreeNode* root, vector<string>& nodes) {
        if (!root) {
            nodes.push_back("#");
            return;
        }
        nodes.push_back(to_string(root->val));
        ser_dfs(root->left, nodes);
        ser_dfs(root->right, nodes);
    }

    // Decodes your encoded data to tree.
    TreeNode* deserialize(string data) {
        vector<string> vals = split(data, '-');
        int i = 0;
        return des_dfs(vals, i);
    }

    TreeNode* des_dfs(vector<string> &v, int &i) {
        if (v[i] == "#") {
            i++;
            return nullptr;
        }
        TreeNode* node = new TreeNode(stoi(v[i]));
        i++;
        node->left = des_dfs(v, i);
        node->right = des_dfs(v, i);
        return node;
    }

    vector<string> split(const string &s, char delim) {
        vector<string> elems;
        stringstream ss(s);
        string item;
        while (getline(ss, item, delim)) {
            elems.push_back(item);
        }
        return elems;
    }
    
    string join(const vector<string> &nodes, const string &delim) {
        ostringstream s;
        for (const auto &i : nodes) {
            if (&i != &nodes[0])
                s << delim;
            s << i;
        }
        return s.str();
    }
};
