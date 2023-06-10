class Solution {
public:
    ListNode* rotateRight(ListNode* head, int k) {
        if (head == nullptr || k == 0) {
            return head;
        }
        
        int count = 1;
        ListNode* tail = head;
        
        while (tail->next != nullptr) {
            tail = tail->next;
            count++;
        }
        
        tail->next = head;
        k %= count;
        
        int steps = count - k;
        ListNode* newTail = head;
        
        while (steps > 1) {
            newTail = newTail->next;
            steps--;
        }
        
        head = newTail->next;
        newTail->next = nullptr;
        
        return head;
    }
};
