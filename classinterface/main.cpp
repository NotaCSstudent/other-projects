#include <iostream>
using namespace std;


struct X
{
    
        
    public:
        int x,y,z;
        X(int a=0,int b=0,int c=0) : x(a),y(b),z(c) {}
};




int main()
{
    X h;
    h.x = 1;
    h.y = 2;
    h.z = 3;
    X t(1,2,3);
    cout << h.x << endl;
    cout << h.y << endl;
    cout << h.z << endl;
    cout << t.x << t.y << t.z << endl;
}




[1,2,3,4]


[1,2],[1,3],[1,4],[2,3],[2,4],[3,4]
[11][12][13][14][21][22][23][24][31][32][33][34][41][42][43][44]
