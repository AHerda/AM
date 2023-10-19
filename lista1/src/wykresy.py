import matplotlib.pyplot as plt
import pandas as pd
import json
import networkx as nx



headers = ["nazwa pliku", "MST size", "MST dfs size", "min from 1000", "avg min from 50", "avg min from 10"]
dane = pd.read_csv("./csv/dane.csv", sep=";", encoding="UTF-16LE", index_col=0, names=headers).transpose()

dane.plot(linestyle="-", marker=".")
plt.xticks(rotation = 15)
plt.savefig("plots/plot.png", dpi=500)
plt.clf()

file_names = ["xqf131", "xqg237", "pma343", "pka379", "bcl380", "pbl395", "pbk411", "pbn423", "pbm436", "xql662"]

for filename in file_names:
    with open(f"./help/graph_{filename}.json") as file:
        dane = json.load(file)
    pos = {}
    for point in dane:
        pos[point["id"]] = (point["x"], point["y"])
    
    G = nx.Graph()
    with open(f"./help/mst_{filename}.json") as file:
        dane = json.load(file)
    for fromm in range(1, len(dane)):
        for too in dane[fromm]:
            G.add_edge(fromm, too)
    
    options = {
    "with_labels": False,
    "node_size": 10,
    "node_color": "red",
    "edgecolors": "red",
    "linewidths": 0,
    "width": 1,
    }
    nx.draw_networkx(G, pos, **options)

    # Set margins for the axes so that nodes aren't clipped
    ax = plt.gca()
    plt.savefig(f"plots/mst_{filename}.png", dpi=500)
    plt.clf()




    G = nx.Graph()
    with open(f"help/dfs_{filename}.json") as file:
        dane = json.load(file)
    last = dane[-1]
    for current in dane:
        G.add_edge(last, current)
        last = current
    
    options = {
    "with_labels": False,
    "node_size": 10,
    "node_color": "red",
    "edgecolors": "red",
    "linewidths": 0,
    "width": 1,
    }
    nx.draw_networkx(G, pos, **options)

    # Set margins for the axes so that nodes aren't clipped
    ax = plt.gca()
    plt.savefig(f"plots/dfs_{filename}.png", dpi=500)
    plt.clf()