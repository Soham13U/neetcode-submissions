class Solution {
public:
    vector<vector<int>> subsets(vector<int>& nums) {
        vector<vector<int>> ans;
        vector<int> temp;
        backtracking(ans,temp,nums,0);

        return ans;
    }

    void backtracking(vector<vector<int>>& ans, vector<int>& temp, vector<int>& nums, int i)
    {
        if( i >= nums.size())
        {
            ans.push_back(temp);
            return;
           
        }

        temp.push_back(nums[i]);
        backtracking(ans,temp,nums,i+1);
        temp.pop_back();
        backtracking(ans,temp,nums,i+1);


    }

};
