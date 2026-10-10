class Solution {
public:
    vector<int> productExceptSelf(vector<int>& nums) {
        vector<int> pre(nums.size(),1);
        vector<int> pos(nums.size(),1);
        vector<int> ans;

        for (int i = 1; i<nums.size(); i++)
        {
            pre[i] = pre[i-1] * nums[i-1];
            
        }
       
        for( int i = nums.size()-2; i>=0; i--)
        {
            pos[i] = pos[i+1] * nums[i+1];
            
        }


        for (int i = 0; i<nums.size(); i++)
        {
            ans.push_back(pre[i] * pos[i]);
        }

        return ans;
    }
};
/*
    1 2 4 6
pre 1 1 2 8
pos 48 24 6 1  
    48,24,12,8
*/