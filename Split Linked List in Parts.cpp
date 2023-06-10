class Solution {
public:
    vector<ListNode*> splitListToParts(ListNode* head, int k) {
        vector<ListNode*> result;
        int listSize = 0;
        ListNode* curr = head;
        
        while (curr) {
            listSize++;
            curr = curr->next;
        }
        
        curr = head;
        int div = listSize / k;
        int rem = listSize % k;
        
        for (int i = 0; i < k; i++) {
            int partSize = div + (i < rem ? 1 : 0);
            ListNode* dummy = new ListNode();
            ListNode* tmp = dummy;
            
            for (int j = 0; j < partSize; j++) {
                tmp->next = curr;
                curr = curr->next;
                tmp = tmp->next;
            }
            
            tmp->next = nullptr;
            result.push_back(dummy->next);
        }
        
        return result;
    }
};
