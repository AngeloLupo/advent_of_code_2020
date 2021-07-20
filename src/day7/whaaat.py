import ipdb


def read_rules():
    with open('src/day7/input') as f:
        input = f.read()
        return input.split("\n")


class Bag:
    def __init__(self, name):
        self.name = name
        self.can_contain = {}
        self.can_be_contained = set()

    def __repr__(self):
        return f"Bag: {self.name}"


def crawl_rules():
    bags_dict = {}
    rules = read_rules()
    for rule in rules:
        name, can_contain = rule.split(" bags contain ")
        # if name == "shiny gold":
        #     import ipdb
        #     ipdb.set_trace()
        if name not in bags_dict:
            bag_class = Bag(name)
            bags_dict[name] = bag_class
        else:
            bag_class = bags_dict[name]

        if "no other" in can_contain:
            continue

        for x in can_contain.split(", "):
            y = x.split(" ")
            quantity = y[0]
            bag_name = f"{y[1]} {y[2]}"
            if bag_name not in bags_dict:
                inner_bag_class = Bag(bag_name)
                bags_dict[bag_name] = inner_bag_class
            else:
                inner_bag_class = bags_dict[bag_name]

            bag_class.can_contain[inner_bag_class.name] = (inner_bag_class, int(quantity))
            inner_bag_class.can_be_contained.add(bag_class)

    return bags_dict


def count_containing_bags(name, bags_dict):
    bag = bags_dict[name]

    containing_bags(bag)


def containing_bags(bag):
    global rr
    for x in bag.can_be_contained:
        rr.add(x)
        containing_bags(x)


def contained_bags(bag):
    return sum(quantity * (1 + contained_bags(inner_bag)) for inner_bag, quantity in bag.can_contain.values())


import sys

rr = set()

#sys.setrecursionlimit(20000)
bags_dict = crawl_rules()

containing_bags(bags_dict["shiny gold"])
print(len(rr))


print(contained_bags(bags_dict["shiny gold"], 0))

#ipdb.set_trace()







