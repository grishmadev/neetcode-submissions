/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */

class Solution {
public:
    ListNode* mergeKLists(vector<ListNode*>& lists) {
        ListNode* head = new ListNode(0);
        ListNode* ptr = head;
        for (auto node: lists) {
            ptr->next = node;
            while (ptr->next) {
                ptr = ptr->next;
            };
        }
        ptr = head->next;
        vector<int> sorted;
        while (ptr) {
            sorted.push_back(ptr->val);
            ptr = ptr->next;
        }
        sort(sorted.begin(), sorted.end());
        ptr = head->next;
        for (auto n : sorted) {
            ptr->val = n;
            ptr = ptr->next;
        }
        return head->next;
    }
};
