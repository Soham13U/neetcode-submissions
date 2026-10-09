class Solution {
public:
    bool isAnagram(string s, string t) {
        map<char,int> map1;
        map<char,int> map2;
        for (char c : s)
        {
            map1[c]++;
        }
        for (char c : t)
        {
            map2[c]++;
        }

        return map1 == map2;
    }
};
