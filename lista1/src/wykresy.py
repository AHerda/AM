import matplotlib.pyplot as plt
import numpy as np
import pandas as pd


headers = ["nazwa pliku", "MST size", "MST dfs size", "min from 1000", "avg min from 50", "avg min from 10"]
dane = pd.read_csv("./plots/dane.csv", sep=";", encoding="UTF-16LE", index_col=0, names=headers).transpose()

dane.plot(linestyle="-", marker=".")
plt.xticks(rotation = 15)
plt.savefig("plots/plot.png", dpi=500)