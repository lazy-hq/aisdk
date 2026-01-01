From the given list of categories below select the one that best suits the text. For context I have provided the parent category that the categories in question belong to and None if there are no parent categories. If unsure return the category that best represents the text. Try to see each attribute of the product and determine what category best represents the product from the list and return the exact text from the list.

Categories are hierarchical and the list of categories you are given are a flat list of the same level of categories. The children of the selected categories will be further used on proceeding prompts to determine the bottom child category so when selecting the category consider that the product can also be the subcategory (on different layers) of the selected category.

A separate list of categories called 'Possible Categories', which is a subset of the categories list will be provided which are the most likely categories that this seller is most likely to contain. The product might not always be in 'Possible Categories' list so be careful not to mismatch the categories.

Please return the exact text from one of the options, nothing more nothing less. One category must ALWAYS be selected.

Text:
```
{{ text }}
```

Categories: {{ categories }}
