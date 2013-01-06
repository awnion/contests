#include <fstream>
#include <set>
#include <vector>
#include <math.h>
#include <stdio.h>

using namespace std;

const int MAXE = 60001;
const int MAXN = 20001;

struct Edge{
	int dist;
	int period;
	int duration;

	Edge(int b, int c, int d)
	{
		this->dist     = b;
		this->period   = c;
		this->duration = d;
	}
};

struct Pair{
	double t;
	int v;

	Pair(double t, int v)
	{
		this->t = t;
		this->v = v;
	}

	int operator < (const Pair &r) const
	{
		if ((this->t == r.t) && (this->v < r.v)) return 1;
		if (this->t < r.t) return 1;
		return 0;
	}
};

typedef vector<Edge> Edges;

int N;

Edges E[MAXE];

double get_path(int x, int y, double curTime)
{
	set<Pair> Time;
	Pair * Vertex[MAXN];
	bool Visited[MAXN];


	for(int i=0; i<=N; i++)
	{
		Vertex[i] = NULL;
		Visited[i] = false;
	}

	Vertex[x] = new Pair(curTime, x);
	Time.insert(*Vertex[x]);

	Pair min((double)-1, -1);

	while (!Time.empty())
	{
		min = *Time.begin(); Time.erase(Time.begin());
		Visited[min.v] = true;
		if (min.v == y) break;

		for (Edges::const_iterator it = E[min.v].begin(); it != E[min.v].end(); it++)
		{
			if (!Visited[it->dist])
			{
				double time = (double)ceil(min.t / it->period) * it->period + it->duration;

				if (Vertex[it->dist] == NULL)
				{
                    Vertex[it->dist] = new Pair(time, it->dist);
					Time.insert(*(Vertex[it->dist]));
				}
				else if ((Vertex[it->dist])->t > time)
				{
					Time.erase(Time.find(*Vertex[it->dist]));
					Vertex[it->dist] = new Pair(time, it->dist);
					Time.insert(*(Vertex[it->dist]));
				}
			}
		}
	}
	
	if (Vertex[y] != NULL) return Vertex[y]->t;
	else return -1;
}

int main()
{
	ifstream fin  ("e.in");
	FILE* fout = fopen("e.out", "w");

	int K, a, b, c, d, M, p1, p2;
	double result = 0;

	fin >> N >> K;

	for (int i = 0; i < K; i++)
	{
		fin >> a >> b >> c >> d;
		E[a].push_back(Edge(b,c,d));
	}

	fin >> M >> p1;

	for (int i = 0; i < M; i++)
	{
		fin >> p2;
		result = get_path(p1, p2, result);
		if (result == -1) break;
		p1 = p2;
	}

    fprintf(fout, "%.0lf", result);

	fin.close ();

	return 0;
}
