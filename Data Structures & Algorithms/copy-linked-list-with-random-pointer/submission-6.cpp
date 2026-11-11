/*
// Definition for a Node.
class Node {
public:
    int val;
    Node* next;
    Node* random;
    
    Node(int _val) {
        val = _val;
        next = NULL;
        random = NULL;
    }
};
*/

class Solution {
public:
    Node* copyRandomList(Node* head) {
        if (!head) return nullptr;
        
        Node* cur = head;
        while (cur) {
            Node* next_node = cur->next;
            Node* copy = new Node(cur->val);
            cur->next = copy;
            copy->next = next_node;
            cur = next_node;
        }

        cur = head;
        while (cur) {
            Node* next = cur->next;
            if (cur->random) {
                next->random = cur->random->next;
            }
            cur = next->next;
        }

        cur = head;
        Node* copyHead = head->next;
        while (cur) {
            Node* copy = cur->next;
            cur->next = copy->next;
            cur = cur->next;
            if (cur) {
                copy->next = cur->next;
            } else {
                copy->next = nullptr;
            }
        }

        return copyHead;
    }
};
