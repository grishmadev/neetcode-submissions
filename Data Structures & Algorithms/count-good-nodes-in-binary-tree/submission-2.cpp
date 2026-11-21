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
    int goodNodes(TreeNode* root) {
        if (!root) return 0;
        int good = 0;
        stack<pair<TreeNode*, int>> s;
        s.push({ root, root->val });
        while (!s.empty()) {
            auto [node, maxVal] = s.top();
            s.pop();
            if (node->val >= maxVal) {
                good++;
                maxVal = node->val;
            };
            if (node->left) s.push({node->left, maxVal});
            if (node->right) s.push({node->right, maxVal});
        }
        return good;
    }
};
