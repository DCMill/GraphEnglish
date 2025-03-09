
import networkx as nx
import matplotlib.pyplot as plt

# Read the graph from a .dot file
G = (nx.nx_pydot.read_dot("../src/output.dot"))

# Draw the graph
nx.draw(G, with_labels=True, node_color='lightblue', font_weight='bold', font_size=10)
plt.show()
