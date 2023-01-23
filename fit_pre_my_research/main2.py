####################################################################################################
# 先行研究(csv)と自身の研究を重ねてプロットするプログラム。
####################################################################################################

import matplotlib.pyplot as plt
import pandas as pd
import math

#---------計算パラメータ----------------------------------------------------------------------------------------
RANGE_BOTTOM    = 100.
RANGE_UPPER     = 250.
#---------自分の計算データ---------------------------------------------------------------------------------
def my_data_read(decaytime_, m_sum_, m_ratio_):
    print("decaytime = %s, m_sum = %s, m_ratio = %s" % (decaytime_, m_sum_, m_ratio_))

    my_data_file = "../../11205to1212/guess_from_m1m2_to_abundance/output/sample2.txt"
    file1 = open(my_data_file, 'r')
    lines = file1.readlines()

    A_my = []
    MF_my = []
    ok = False

    europium151 = 0.0
    europium153 = 0.0
    thorium232 = 0.0
    uranium235 = 0.0
    uranium238 = 0.0


    for (i, line) in enumerate(lines):
        if '#' in line.split(" ")[0]:
            line = line.split(" ")
            decaytime   = line[2].split(",")[0]
            m_sum       = line[5].split(",")[0]
            m_ratio     = line[8].split("\n")[0]
            # print(decaytime, m_sum, m_ratio)
            if decaytime == decaytime_ and m_sum == m_sum_ and m_ratio == m_ratio_:
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
                if float(a) >= RANGE_BOTTOM:
                    A_my.append(float(a))
                    MF_my.append(float(mf))
                if z == "63" and a == "151":
                    europium151 = float(mf)
                if z == "63" and a == "153":
                    europium153 = float(mf)
                if z == "90" and a == "232":
                    thorium232 = float(mf)
                if z == "92" and a == "235":
                    uranium235 = float(mf)
                if z == "92" and a == "238":
                    uranium238 = float(mf)
    print("Eu151 = %.4e"% europium151)
    print("Eu153 = %.4e"% europium153)
    print("Th252 = %.4e"% thorium232)
    print("U235 = %.4e"% uranium235)
    print("U238 = %.4e"% uranium238)
    if europium151 + europium153 != 0.0:
        print("Th232/Eu = %f" % (thorium232 / (europium151 + europium153)))
    if uranium235 + uranium238 != 0.0:
        print("Th232/U = %f" % (thorium232 / (uranium235 + uranium238)))

    base = sum(MF_my)
    for i in range(len(MF_my)):
        MF_my[i] = MF_my[i] / base
    print("\n")
    return (A_my, MF_my)

#--------論文のデータ---------------------------------------------------------------------------------

def convert_abundance_to_mf(x, y):
    def search_massamu(x, A, mass_amu):
        idx = -1
        dif = 1001001001.
        for i in range(len(A)):
            if dif > abs(x - A[i]):
                idx = i
                dif = abs(x - A[i])
        
        if idx == -1 or dif >= 1e2:
            print("hogeeeeeeee")
            return 0
        return mass_amu[idx]
    
    f = open("./input/solar_abundance.txt", 'r')
    lines = f.readlines()
    
    A = []
    mass_amu = []
    for (i, line) in enumerate(lines):
        if i == 0:
            continue
        _, _, a, m, _ = line.split("\t")
        A.append(int(a))
        mass_amu.append(float(m))

    for i in range(len(x)):
        y[i] = y[i] * search_massamu(x[i], A, mass_amu)
    
    base = sum(y)
    for i in range(len(y)):
        y[i] /= base
    return (x, y)


def read_presearch(papernum, fignum, figposition, condition):
    presearch_datafile = "../mf_papers/" + papernum + fignum + figposition + ".csv"
    
    data = pd.read_csv(presearch_datafile, header = [0, 1])

    data_x = data[:][condition]['X'].to_list()
    data_y = data[:][condition]['Y'].to_list()

    data_x_ = []
    data_y_ = []
    for i in range(len(data_y)):
        if not math.isnan(data_y[i]) and data_x[i] >= RANGE_BOTTOM:
            data_x_.append(data_x[i])
            data_y_.append(data_y[i])
       
        # (data_x_, data_y_) = convert_abundance_to_mf(data_x_, data_y_)

    base = sum(data_y_)
    print(base)
    for i in range(len(data_y_)):
        data_y_[i] = data_y_[i]/ base
    if len(data_x) < 10:
        print(papernum, fignum, figposition, condition)
    return (data_x_, data_y_)

def presearch_plot(papernum, fignum, figposition, condition):
    (data_x, data_y) = read_presearch(papernum, fignum, figposition, condition)
    tmp = ""
    if papernum == "systematic01":
        tmp = "Bauswein et al.(2013) (" + condition + ")"#+ fignum.split("_")[1]
    elif papernum == "systematic02" :
        tmp = "Bovard et al. (2017) (" + condition + ")" #+ fignum.split("_")[1]
    elif papernum == "systematic04":
        tmp = "Fujibayashi et al. (2022) (" + condition + ")" #+ fignum.split("_")[1]
    elif papernum == "systematic06":
        tmp = "Radice et al.(2018) (" + condition + ")" #+ fignum.split("_")[1]
    elif papernum == "systematic08":
        tmp = "Just et al.(2014) (" + condition + ")"
    elif papernum == "systematic09":
        tmp = "Korobkin et al.(2012) (" + condition + ")"
    elif papernum == "systematic11":
        tmp = "Kullmann et al.(2022) (" + condition + ")" #+ fignum.split("_")[1]
    elif papernum == "systematic14":
        tmp = "Papenfort et al. (2018) (" + condition + ")" #+ fignum.split("_")[1]
    elif papernum == "systematic17":
        tmp = "Rosswog et al. (2014) (" + condition + ")"
    plt.plot(data_x, data_y, label = tmp)
    # plt.plot(data_x, data_y, label = papernum + fignum +figposition + condition)

#--------太陽系---------------------------------------------------------------------------------
def plot_solarmf():
    solarmf_file = open("./input/solar_abundance.txt", 'r')
    lines = solarmf_file.readlines()

    smf_x = []
    smf_y = []

    for (i,line) in enumerate(lines):
        if i == 0:
            continue
        e, z, a, mass_amu, y = line.split("\t")
        if float(a) >= RANGE_BOTTOM:
            smf_x.append(int(a))
            smf_y.append(float(y) * float(mass_amu))

    base = sum(smf_y)
    for i in range(len(smf_y)):
        smf_y[i] = smf_y[i] / base
    plt.scatter(smf_x, smf_y, label = "solarmf")

#====================================================================================================
# 初期設定
plt.figure(figsize = (20, 10))

#-------solarmfをプロット------------------------------------------------------------------------------
plot_solarmf()

#-------自分のデータをプロット------------------------------------------------------------------------------
###########m_ratio = 1##############
# m1 = 1.25, M2 = 1.25,     m_sum = 2.5,    m_ratio = 1
(A_my, MF_my) = my_data_read("1e17", "2.5", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.5_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.25, M_2 = 1.25$")

# # m1 = 1.35, m2 = 1.35,   m_sum = 2.7,    m_ratio = 1
(A_my, MF_my) = my_data_read("1e17", "2.7", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.35, M_2 = 1.35$")

# m1 = 1.40, M2 = 1.40,   m_sum = 2.8,    m_ratio = 1
(A_my, MF_my) = my_data_read("1e17", "2.8", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.8_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.40, M_2 = 1.40$")

# m1 = 1.45, M2 = 1.45,     m_sum = 2.9,    m_ratio = 1
(A_my, MF_my) = my_data_read("1e17", "2.9", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.9_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.45, M_2 = 1.45$")

# m1 = 1.50, M2 = 1.50,     m_sum = 3.0,    m_ratio = 1
(A_my, MF_my) = my_data_read("1e17", "3", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_3_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.50, M_2 = 1.50$")

############m_sum = 2.7############
# m1 = 1.2, M2 = 1.5,       m_sum = 2.7,    m_ratio = 0.8
(A_my, MF_my) = my_data_read("1e17", "2.7", "0.8")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_0.8")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.50$")

# m1 = 1.25, M2 = 1.45,     m_sum = 2.7,    m_ratio = 0.86
(A_my, MF_my) = my_data_read("1e17", "2.7", "0.86")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_0.86")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.25, M_2 = 1.45$")

# m1 = 1.30, M2 = 1.40,     m_sum = 2.7,    m_ratio = 0.93
(A_my, MF_my) = my_data_read("1e17", "2.7", "0.92")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_0.92")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.30, M_2 = 1.40$")

# # m1 = 1.35, m2 = 1.35,   m_sum = 2.7,    m_ratio = 1
# (A_my, MF_my) = my_data_read("1e17", "2.7", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.35, M_2 = 1.35$")


############m_sum = 3.0############
# # m1 = 1.20, M2 = 1.80,   m_sum = 3.0,    m_ratio = 0.67
(A_my, MF_my) = my_data_read("1e17", "3", "0.67")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_3_mratio_0.67")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.80$")

# m1 = 1.50, M2 = 1.50,     m_sum = 3.0,    m_ratio = 1
# (A_my, MF_my) = my_data_read("1e17", "3", "1")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_3_mratio_1")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.50, M_2 = 1.50$")

############m_ratio = 0.8############
# m1 = 1.25, M2 = 1.55,     m_sum = 2.8,    m_ratio = 0.80
(A_my, MF_my) = my_data_read("1e17", "2.8", "0.8")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.8_mratio_0.8")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.25, M_2 = 1.55$")

# m1 = 1.2, M2 = 1.5,       m_sum = 2.7,    m_ratio = 0.8
# (A_my, MF_my) = my_data_read("1e17", "2.7", "0.8")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_0.8")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.50$")

############m1 = 1.2############
# # m1 = 1.20, M2 = 1.40,   m_sum = 2.6,    m_ratio = 0.86
(A_my, MF_my) = my_data_read("1e17", "2.6", "0.86")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.6_mratio_0.86")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.40$")

# m1 = 1.2, M2 = 1.5,       m_sum = 2.7,    m_ratio = 0.8
# (A_my, MF_my) = my_data_read("1e17", "2.7", "0.8")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_2.7_mratio_0.8")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.50$")

# m1 = 1.20, M2 = 1.60,     m_sum = 2.8,    m_ratio = 0.75
(A_my, MF_my) = my_data_read("1e17", "2.8", "0.75")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_3_mratio_0.67")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.60$")

# # m1 = 1.20, M2 = 1.80,   m_sum = 3.0,    m_ratio = 0.67
# (A_my, MF_my) = my_data_read("1e17", "3", "0.67")
# plt.plot(A_my, MF_my, label = "decaytime_1e15_msum_3_mratio_0.67")
# plt.plot(A_my, MF_my, label = "decaytime = $10^{17}$, $M_1 = 1.20, M_2 = 1.80$")

#-------先行研究をプロット------------------------------------------------------------------------------
# m1 = 1.35, m2 = 1.35
# presearch_plot("systematic01", "_fig08", "", "DD2")
# presearch_plot("systematic01", "_fig08", "", "NL3")
# presearch_plot("systematic01", "_fig08", "", "SFHo")
# presearch_plot("systematic01", "_fig10", "", "BSk20")
# presearch_plot("systematic01", "_fig10", "", "BSk21")

# presearch_plot("systematic02", "_fig08", "", "LS220-M135")
# presearch_plot("systematic02", "_fig09", "_left", "DD2-M1.35")
# presearch_plot("systematic02", "_fig09", "_middle", "LS220-M1.35")
# presearch_plot("systematic02", "_fig09", "_right", "SFHo-M1.35")

# presearch_plot("systematic04", "_fig05", "_left", "SFHo-135-135")
# presearch_plot("systematic04", "_fig05", "_right", "DD2-135")

# presearch_plot("systematic06", "_fig18", "_right", "LS220_M135135_LK")
# presearch_plot("systematic06", "_fig18", "_right", "LS220_M135135_M0_LTE")
# presearch_plot("systematic06", "_fig18", "_right", "LS220_M135135_M0")
# presearch_plot("systematic06", "_fig19", "_right", "LS220_M135135_M0")
# presearch_plot("systematic06", "_fig20", "_right", "BHBlp_M135135_M0")
# presearch_plot("systematic06", "_fig20", "_right", "DD2_M135135_m0")
# presearch_plot("systematic06", "_fig20", "_right", "LS220_M135135_M0")
# presearch_plot("systematic06", "_fig20", "_right", "SFHoM135135_M0")

# presearch_plot("systematic10", "_fig04", "", "DD2_135-135")
# presearch_plot("systematic10", "_fig04", "", "SFHo_135-135")
# presearch_plot("systematic11", "_fig07", "_middle", "BSkG2")
# presearch_plot("systematic11", "_fig07", "_middle", "D1M")
# presearch_plot("systematic11", "_fig07", "_middle", "HFB-31")
# presearch_plot("systematic11", "_fig07", "_middle", "FRDM12")
# presearch_plot("systematic11", "_fig07", "_middle", "HFB-21")
# presearch_plot("systematic11", "_fig07", "_middle", "WS4")

# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP11.250")
# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP12.500")
# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP13.000")
# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP13.250")
# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP13.375")
# presearch_plot("systematic14", "_fig08", "_topleft", "DD2-RP14.000")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP11.250")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP12.500")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP13.000")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP13.250")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP13.340")
# presearch_plot("systematic14", "_fig08", "_topright", "LS220-RP13.375")


# m1 = 1.2, m2 = 1.5
# presearch_plot("systematic01", "_fig09", "", "DD2")
# presearch_plot("systematic01", "_fig09", "", "NL3")
# presearch_plot("systematic01", "_fig09", "", "SFHo")
# presearch_plot("systematic04", "_fig05", "_middle", "SFHo120-150")
# presearch_plot("systematic04", "_fig06", "", "SFHo120-150")

# m1 = 1.25, m2 = 1.25
# presearch_plot("systematic02", "_fig09", "_left", "DD2-M1.25")
# presearch_plot("systematic02", "_fig09", "_middle", "LS220-M1.25")
# presearch_plot("systematic02", "_fig09", "_right", "SFHo-M1.25")

# # m1 = 1.45, m2 = 1.45
# presearch_plot("systematic02", "_fig09", "_left", "DD2-M1.45")
# presearch_plot("systematic02", "_fig09", "_middle", "LS220-M1.45")
# presearch_plot("systematic02", "_fig09", "_right", "SFHo-M1.45")
# presearch_plot("systematic08", "_fig11", "", "SFHo145-145")

# m1 = 1.30, m2 = 1.40
# presearch_plot("systematic04", "_fig06", "", "SFHo130-140")
# presearch_plot("systematic17", "_fig10", "_topright", "1.3-1.4_Mwind_1e-2")
# presearch_plot("systematic17", "_fig10", "_topright", "1.3-1.4_Mwind_1e-3")
# presearch_plot("systematic17", "_fig10", "_topright", "1.3-1.4_Mwind_1e-4")

# m1 = 1.25, m2 = 1.45
# presearch_plot("systematic04", "_fig06", "", "SFHo125-145")
# presearch_plot("systematic10", "_fig04", "", "DD2_125-145")
# presearch_plot("systematic10", "_fig04", "", "SFHo_125-145")
# presearch_plot("systematic11", "_fig07", "_top", "BSkG2")
# presearch_plot("systematic11", "_fig07", "_top", "D1M")
# presearch_plot("systematic11", "_fig07", "_top", "HFB-31")
# presearch_plot("systematic11", "_fig07", "_top", "FRDM12")
# presearch_plot("systematic11", "_fig07", "_top", "HFB-21")
# presearch_plot("systematic11", "_fig07", "_top", "WS4")

# m1 = 1.25, m2 = 1.55
# presearch_plot("systematic04", "_fig06", "", "SFHo125-155")

# m1 = 1.20, m2 = 1.40
# presearch_plot("systematic06", "_fig19", "_right", "LS220_M140120_M0")

# m1 = 1.20, m2 = 1.80
# presearch_plot("systematic08", "_fig10", "", "SFHo120-180")
# presearch_plot("systematic17", "_fig10", "_bottomright", "1.8-1.2_Mwind_1e-2")
# presearch_plot("systematic17", "_fig10", "_bottomright", "1.8-1.2_Mwind_1e-3")
# presearch_plot("systematic17", "_fig10", "_bottomright", "1.8-1.2_Mwind_1e-4")

# m1 = 1.50, m2 = 1.50
# presearch_plot("systematic08", "_fig11", "", "SFHX150-150")

# m1 = 1.40, m2 = 1.40
# presearch_plot("systematic09", "_fig04", "_bottom", "SFHX150-150")
# presearch_plot("systematic17", "_fig10", "_topleft", "1.4-1.4_Mwind_1e-2")
# presearch_plot("systematic17", "_fig10", "_topleft", "1.4-1.4_Mwind_1e-3")
# presearch_plot("systematic17", "_fig10", "_topleft", "1.4-1.4_Mwind_1e-4")

# # m1 = 1.20, m2 = 1.60
# presearch_plot("systematic17", "_fig10", "_bottomleft", "1.6-1.2_Mwind_1e-2")
# presearch_plot("systematic17", "_fig10", "_bottomleft", "1.6-1.2_Mwind_1e-3")
# presearch_plot("systematic17", "_fig10", "_bottomleft", "1.6-1.2_Mwind_1e-4")

#====================================================================================================
# ------------図の設定--------------------------------------------------------------------------------
# plt.legend(bbox_to_anchor=(1, 0), loc='lower right', borderaxespad=1, fontsize = 18)
plt.xlim([RANGE_BOTTOM, RANGE_UPPER])
# plt.xlim([0, 250])
plt.ylim([1e-10, 1])
plt.yscale('log')
plt.xlabel("A", fontsize = 18)
plt.ylabel("Mass Fraction", fontsize = 18)
# plt.savefig("../../11226to0109/修論画像作り/last_abundance/abundance_M1_120.png")
# plt.savefig("./pic/120-160_ver2.png")
# plt.show()
