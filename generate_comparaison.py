import csv
import matplotlib.pyplot as plt

with open("methods_comparaison.csv") as data:
    read = csv.reader(data)
    
    for i, row in enumerate(read):
        if not row:
            continue
        
        label = row[0]
        Y = list(map(float, row[1:])) 

        
        X = range(len(Y))
        plt.plot(X, Y, label=label)

    plt.xlabel("Itérations")
    plt.ylabel("Distance totale")
    plt.grid(True)
    plt.legend()
    plt.title("Évolution de la meilleure distance trouvée")
    plt.show()
