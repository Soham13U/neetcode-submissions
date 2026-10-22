class Solution {
public:
    vector<vector<int>> combinationSum(vector<int>& nums, int target) {
        vector<vector<int>> ans;
        vector<int> temp;

        backtrack(ans,temp,nums,target,0,0);

        return ans;
    }

    void backtrack(vector<vector<int>>& ans, vector<int>& temp, vector<int>& nums, int target, int sum, int i)
    {
        if(sum > target || i>= nums.size())
        {
            
            return;
        }
        else if(sum == target)
        {
            ans.push_back(temp);
            return;
        }

        temp.push_back(nums[i]);
        sum += nums[i];
        backtrack(ans,temp,nums,target,sum,i);
        temp.pop_back();
        sum -= nums[i];
        backtrack(ans,temp,nums,target,sum,i+1);

    }
};

/*
2 i
5 i+2

sum > target
pop

sum == target
push
*/
