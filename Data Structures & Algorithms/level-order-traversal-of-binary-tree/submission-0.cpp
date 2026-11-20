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

class Solution {
public:
    vector<vector<int>> levelOrder(TreeNode* root) {
        vector<vector<int>> res;
        if (!root) return res;
        deque<pair<TreeNode*, int>> dq;
        dq.push_back({ root, 0 });
        while (!dq.empty()) {
            auto elem = dq.front();
            TreeNode* node = elem.first;
            int lvl = elem.second;
            dq.pop_front();
            if (lvl == (int)res.size()) res.push_back({});
            res[lvl].push_back(node->val);
            if (node->left) dq.push_back({node->left, lvl + 1});
            if (node->right) dq.push_back({node->right, lvl + 1});
        }
        return res;
    }
};
