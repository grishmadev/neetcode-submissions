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
    ListNode* reverseKGroup(ListNode* head, int k) {
        int count = 0;
        ListNode* ptr = head;
        ListNode* dummy = new ListNode(0);
        ListNode* res = dummy;
        
        while (ptr) {
            ptr = ptr->next;
            count += 1;
        }
        ptr = head;
        for (int i = 0; i + k <= count; i += k) {
            ListNode* cursor = ptr;
            ListNode* end = ptr;
            for (int j = 0; j < k - 1; j++) {
                end = end->next;
            }
            ListNode* temp = end->next;
            end->next = nullptr;
            ListNode* reversed = reverse(cursor);
            res->next = reversed;

            cursor->next = temp;

            res = cursor;
            ptr = temp;
        }
        return dummy->next;
    }

private:
    ListNode* reverse(ListNode* head) {
        ListNode* prev = nullptr;
        ListNode* cur = head;
        while (cur) {
            ListNode* next = cur->next;
            cur->next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }
};
