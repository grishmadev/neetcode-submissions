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
    int maxDepth(TreeNode* root) {
        // iterative depth first search
        stack<pair<TreeNode*, int>> stk;
        stk.push({root, 1});
        int res = 0;
        while (!stk.empty()) {
            TreeNode* node = stk.top().first;
            int lvl = stk.top().second;
            stk.pop();
            if (node) {
                res = max(res, lvl);
                if (node->right) stk.push({node->right, lvl + 1});
                if (node->left) stk.push({node->left, lvl + 1});
            }
        }
        return res;
    }
};
