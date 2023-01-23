####################################################################################################
# SystematicPapers のm1,m2に結びついているmfと自分で計算したmfを合わせてフィットする。
####################################################################################################

import matplotlib.pyplot as plt
import pandas as pd


# --------自分で計算したデータをimport------------------------------------------------------------------
my_data_file = "../../11205to1212/guess_from_m1m2_to_abundance/output/sample2.txt"
file1 = open(my_data_file, 'r')
lines = file1.readlines()

A_my = []
MF_my = []
ok = False


for (i, line) in enumerate(lines):
    if '#' in line.split(" ")[0]:
        line = line.split(" ")
        decaytime   = line[2].split(",")[0]
        m_sum       = line[5].split(",")[0]
        m_ratio     = line[8].split("\n")[0]
        # print(decaytime, m_sum, m_ratio)
        if decaytime == "1e15" and m_sum == "3" and m_ratio == "1":
            ok = True
        else:
            ok = False
        continue

    if ok:
        line = line.split("\t")
        if len(line) == 4:
            z, a, n, mf = line
            if float(mf) <= 0.:
                continue
            A_my.append(float(a))
            MF_my.append(float(mf))

base = sum(MF_my)
for i in range(len(MF_my)):
    MF_my[i] = MF_my[i] / base

fig = plt.figure()
plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_1")
# --------先行研究のグラフデータをimport------------------------------------------------------------------
presearch_data_file = "../mf_papers/systematic08_fig10.csv"

data = pd.read_csv(presearch_data_file, header = [0, 1])

condition_list = set([])
for column in data.columns:
    condition, axis_name = column
    if condition in condition_list:
        continue
    condition_list.add(condition)
    data_x = data[:][condition]['X'].tolist()
    data_y = data[:][condition]['Y'].tolist()

    base = sum(data_y)
    for i in range(len(data_y)):
        data_y[i] = data_y[i] / base
    plt.plot(data_x, data_y, label = "systematic08_fig10_" + condition)

plt.savefig("./pic/sys08_fig10_135-135.png")
plt.legend()
plt.yscale('log')
plt.xlabel("A")
plt.ylabel("Mass Fraction")
plt.show()

