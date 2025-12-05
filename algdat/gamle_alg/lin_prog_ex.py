import pulp as p

# Definer problemet
model = p.LpProblem('HammersAndNails', p.LpMaximize)

# Her skal du definere beslutningsvariablene

# Eksempel:
x1 = p.LpVariable("x1", lowBound = 0)
x2 = p.LpVariable("x2", lowBound = 0)

# Her skal du legge til den linÃ¦re funksjonen som skal optimeres

# Eksempel:
model += 3000*x1 +1000*x2

# Her skal du legge til de lineÃ¦re ulikhetene som pÃ¥ oppfylles

# Eksempel:
# model += x + y <= 30
model += 2*x1+x2<=100
model += x1<=40
model += x1 + 2*x2 <= 80
# Print modellen
print(model)

# LÃ¸s det lineÃ¦re programmet
status = model.solve()

# Print lÃ¸sningen

# Print verdien til en beslutningsvariabel:
print(p.value(x1))
print(p.value(x2))

# Print optimal verdi:
print(p.value(model.objective))
