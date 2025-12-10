---
title: JavaScript Reactivity for Beginners
description: A beginner-friendly guide to understanding reactivity in JavaScript
---

# JavaScript Reactivity for Beginners

## 1. Basic JavaScript Variables

Let's start with the simplest form of data storage in JavaScript:

```javascript
let price = 10;
let quantity = 2;
let total = price * quantity;

console.log(total); // 20

// If we change price
price = 20;
console.log(total); // Still 20! Not what we want
```

The problem: When we change `price`, `total` doesn't automatically update. This is because JavaScript is not reactive by default.

## 2. Making It Reactive with Functions

The simplest way to make something reactive is to use functions:

```javascript
let price = 10;
let quantity = 2;

function calculateTotal() {
  return price * quantity;
}

console.log(calculateTotal()); // 20

price = 20;
console.log(calculateTotal()); // 40 - Now it updates!
```

This works, but we have to manually call `calculateTotal()` every time we want the updated value.

## 3. Using Object Properties (Getters and Setters)

We can use JavaScript's `Object.defineProperty` to create reactive properties:

```javascript
let data = {
  price: 10,
  quantity: 2,
};

let total = 0;

// Create a reactive property
Object.defineProperty(data, "total", {
  get() {
    return this.price * this.quantity;
  },
});

console.log(data.total); // 20

data.price = 20;
console.log(data.total); // 40 - Automatically updates!
```

## 4. Using Proxy (Modern JavaScript)

The `Proxy` object is a more modern way to make objects reactive:

```javascript
let data = {
  price: 10,
  quantity: 2,
};

// Create a proxy
let proxy = new Proxy(data, {
  get(target, property) {
    console.log(`Getting ${property}`);
    return target[property];
  },
  set(target, property, value) {
    console.log(`Setting ${property} to ${value}`);
    target[property] = value;
    return true;
  },
});

console.log(proxy.price); // Logs: "Getting price" then 10
proxy.price = 20; // Logs: "Setting price to 20"
```

## 5. Creating a Simple Reactivity System

Let's build a basic reactivity system step by step:

```javascript
// 1. Create a dependency tracker
class Dep {
  constructor() {
    this.subscribers = new Set();
  }

  // Add a subscriber
  depend() {
    if (activeEffect) {
      this.subscribers.add(activeEffect);
    }
  }

  // Notify all subscribers
  notify() {
    this.subscribers.forEach((effect) => effect());
  }
}

// 2. Create a reactive function
let activeEffect = null;

function watchEffect(effect) {
  activeEffect = effect;
  effect();
  activeEffect = null;
}

// 3. Make an object reactive
function reactive(obj) {
  const deps = new Map();

  return new Proxy(obj, {
    get(target, key) {
      if (activeEffect) {
        let dep = deps.get(key);
        if (!dep) {
          dep = new Dep();
          deps.set(key, dep);
        }
        dep.depend();
      }
      return target[key];
    },
    set(target, key, value) {
      target[key] = value;
      const dep = deps.get(key);
      if (dep) {
        dep.notify();
      }
      return true;
    },
  });
}

// 4. Use our reactive system
const state = reactive({
  price: 10,
  quantity: 2,
});

// Watch for changes
watchEffect(() => {
  console.log(`Total: ${state.price * state.quantity}`);
});

// Change values
state.price = 20; // Logs: "Total: 40"
state.quantity = 3; // Logs: "Total: 60"
```

## 6. Real-World Example: Shopping Cart

Let's see how this works in a practical example:

```javascript
// Create a reactive shopping cart
const cart = reactive({
  items: [],
  total: 0,
});

// Watch for changes in the cart
watchEffect(() => {
  cart.total = cart.items.reduce((sum, item) => sum + item.price, 0);
  console.log(`Cart total: $${cart.total}`);
});

// Add items to cart
cart.items.push({ name: "Apple", price: 1 });
cart.items.push({ name: "Banana", price: 2 });
// Logs: "Cart total: $3"

// Update item price
cart.items[0].price = 2;
// Logs: "Cart total: $4"
```

## 7. Common Patterns in Modern Frameworks

### Vue.js Style

```javascript
const state = reactive({
  count: 0,
});

watchEffect(() => {
  document.getElementById("counter").textContent = state.count;
});

// Update will automatically reflect in the DOM
state.count++;
```

### React Style

```javascript
let state = {
  count: 0,
};

function setState(newState) {
  state = { ...state, ...newState };
  render();
}

function render() {
  document.getElementById("counter").textContent = state.count;
}

// Update will trigger a re-render
setState({ count: state.count + 1 });
```

## Key Concepts to Remember

1. **Reactivity** means automatically updating when data changes
2. **Dependencies** are tracked when data is accessed
3. **Effects** are functions that run when dependencies change
4. **Proxies** and **Getters/Setters** are tools to make objects reactive
5. **State Management** is about keeping track of changes

## Common Pitfalls

1. **Circular Dependencies**

```javascript
// Avoid this!
watchEffect(() => {
  state.a = state.b + 1;
});

watchEffect(() => {
  state.b = state.a + 1;
});
```

2. **Memory Leaks**

```javascript
// Always clean up effects when done
const stop = watchEffect(() => {
  // effect code
});

// When component unmounts
stop();
```
