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
    int preidx = 0;
    int inidx = 0;
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        return dfs(preorder, inorder, INT_MAX);
    }
    TreeNode* dfs(vector<int>& preorder, vector<int>& inorder, int limit) {
        if (preidx >= preorder.size()) return nullptr;
        if (inorder[inidx] == limit) {
            inidx++;
            return nullptr;
        }
        TreeNode* root = new TreeNode(preorder[preidx++]);
        root->left = dfs(preorder, inorder, root->val);
        root->right = dfs(preorder, inorder, limit);
        return root;
    }
};
