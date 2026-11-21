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
    bool isValidBST(TreeNode* root) {
        if (!root) return true;
        stack<TreeNode*> s;
        s.push(root);
        while (!s.empty()) {
            auto node = s.top();
            s.pop();
            if (node->left) {
                if (node->val < node->left->val) return false;
                s.push(node->left);
            }
            if (node->right) {
                if (node->val > node->right->val) return false;
                s.push(node->right);
            }
        }
        return true;
    }
};
