/**
 * Definition for a binary tree node.
 * class TreeNode {
 *     constructor(val = 0, left = null, right = null) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
type Dfs = {
    balanced: boolean,
    level: number
}

class Solution {
    isBalanced(root: TreeNode | null): boolean {
        return this.dfs(root).balanced;
    }

    dfs(root: TreeNode | null): Dfs {
        if (!root) return {
            balanced: true,
            level: 0
        };
        const left = this.dfs(root.left);
        const right = this.dfs(root.right);
        return {
            balanced: left.balanced && right.balanced && Math.abs(left.level - right.level) <= 1,
            level: 1 + Math.max(left.level, right.level)
        };
    }
}
