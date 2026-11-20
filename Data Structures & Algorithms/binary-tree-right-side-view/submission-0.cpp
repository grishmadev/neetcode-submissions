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
    vector<int> rightSideView(TreeNode* root) {
        vector<int> res;
        if (!root) return res;
        queue<pair<TreeNode*, int>> q;
        q.push({root, 0});
        while (!q.empty()) {
            auto elem = q.front();
            int target = 0;
            while (!q.empty() && q.front().second == elem.second) {
                auto [node, lvl] = q.front();
                q.pop();
                target = node->val;
                if (node->left) q.push({ node->left, lvl + 1 });
                if (node->right) q.push({ node->right, lvl + 1 });
            }
            res.push_back(target);
        }
        return res;
    }
};
