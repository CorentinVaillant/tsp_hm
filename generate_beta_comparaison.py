import csv
import matplotlib.pyplot as plt

with open("beta_comparaison.csv") as data:
    reader = csv.reader(data)

    X = next(reader)[1:] #Beta
    X = list(map(float,X))
    for i,row in enumerate(reader):
        if not row:
            continue

        Y =list(map(float,row[1:]))

        plt.plot(X, Y, label=row[0]) 

    plt.xlabel("Beta")
    plt.ylabel("Distance totale")
    plt.grid(True)
    plt.legend()
    plt.title("Distance trouvée selon beta")
    plt.show()
