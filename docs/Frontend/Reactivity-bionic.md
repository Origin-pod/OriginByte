---
title: JavaScript Reactivity for Beginners
description: A beginner-friendly guide to understanding reactivity in JavaScript
---

# **Jav**aScript **Rea**ctivity for **Beg**inners

## 1. **Ba**sic **Jav**aScript **Var**iables

Let's **st**art **wi**th the **sim**plest **fo**rm of **da**ta **st**orage in **Jav**aScript:

```**jav**ascript
let **pr**ice = 10;
let **qua**ntity = 2;
let **to**tal = **pr**ice * **qua**ntity;

console.log(total); // 20

// If we **ch**ange **pr**ice
**pr**ice = 20;
console.log(total); // **St**ill 20! Not **wh**at we **wa**nt
```

The **pr**oblem: **Wh**en we **ch**ange `**pr**ice`, `**to**tal` doesn't **aut**omatically **up**date. **Th**is is **be**cause **Jav**aScript is not **rea**ctive by **de**fault.

## 2. **Ma**king It **Rea**ctive **wi**th **Fun**ctions

The **sim**plest way to **ma**ke **som**ething **rea**ctive is to use **fun**ctions:

```**jav**ascript
let **pr**ice = 10;
let **qua**ntity = 2;

**fun**ction **cal**culateTotal() {
  **re**turn **pr**ice * **qua**ntity;
}

console.log(calculateTotal()); // 20

**pr**ice = 20;
console.log(calculateTotal()); // 40 - Now it **up**dates!
```

**Th**is **wo**rks, but we **ha**ve to **man**ually **ca**ll `**cal**culateTotal()` **ev**ery **ti**me we **wa**nt the **up**dated **va**lue.

## 3. **Us**ing **Ob**ject **Pro**perties (**Ge**tters and **Se**tters)

We can use JavaScript's `Object.defineProperty` to **cr**eate **rea**ctive **pro**perties:

```**jav**ascript
let **da**ta = {
  **pr**ice: 10,
  **qua**ntity: 2,
};

let **to**tal = 0;

// **Cr**eate a **rea**ctive **pro**perty
Object.defineProperty(data, "**to**tal", {
  get() {
    **re**turn this.price * this.quantity;
  },
});

console.log(data.total); // 20

data.price = 20;
console.log(data.total); // 40 - **Aut**omatically **up**dates!
```

## 4. **Us**ing **Pr**oxy (**Mo**dern **Jav**aScript)

The `**Pr**oxy` **ob**ject is a **mo**re **mo**dern way to **ma**ke **ob**jects **rea**ctive:

```**jav**ascript
let **da**ta = {
  **pr**ice: 10,
  **qua**ntity: 2,
};

// **Cr**eate a **pr**oxy
let **pr**oxy = new Proxy(data, {
  get(target, **pro**perty) {
    console.log(`Getting ${**pro**perty}`);
    **re**turn target[property];
  },
  set(target, **pro**perty, **va**lue) {
    console.log(`Setting ${**pro**perty} to ${**va**lue}`);
    target[property] = **va**lue;
    **re**turn **tr**ue;
  },
});

console.log(proxy.price); // **Lo**gs: "**Ge**tting **pr**ice" **th**en 10
proxy.price = 20; // **Lo**gs: "**Se**tting **pr**ice to 20"
```

## 5. **Cre**ating a **Si**mple **Rea**ctivity **Sy**stem

Let's **bu**ild a **ba**sic **rea**ctivity **sy**stem **st**ep by **st**ep:

```**jav**ascript
// 1. **Cr**eate a **dep**endency **tr**acker
**cl**ass Dep {
  **con**structor() {
    this.subscribers = new Set();
  }

  // Add a **sub**scriber
  **de**pend() {
    if (**act**iveEffect) {
      this.subscribers.add(activeEffect);
    }
  }

  // **No**tify all **sub**scribers
  **no**tify() {
    this.subscribers.forEach((effect) => **ef**fect());
  }
}

// 2. **Cr**eate a **rea**ctive **fun**ction
let **act**iveEffect = **nu**ll;

**fun**ction watchEffect(effect) {
  **act**iveEffect = **ef**fect;
  **ef**fect();
  **act**iveEffect = **nu**ll;
}

// 3. **Ma**ke an **ob**ject **rea**ctive
**fun**ction reactive(obj) {
  **co**nst **de**ps = new Map();

  **re**turn new Proxy(obj, {
    get(target, key) {
      if (**act**iveEffect) {
        let dep = deps.get(key);
        if (!dep) {
          dep = new Dep();
          deps.set(key, dep);
        }
        dep.depend();
      }
      **re**turn target[key];
    },
    set(target, key, **va**lue) {
      target[key] = **va**lue;
      **co**nst dep = deps.get(key);
      if (dep) {
        dep.notify();
      }
      **re**turn **tr**ue;
    },
  });
}

// 4. Use our **rea**ctive **sy**stem
**co**nst **st**ate = **rea**ctive({
  **pr**ice: 10,
  **qua**ntity: 2,
});

// **Wa**tch for **ch**anges
**wat**chEffect(() => {
  console.log(`Total: ${state.price * state.quantity}`);
});

// **Ch**ange **va**lues
state.price = 20; // **Lo**gs: "**To**tal: 40"
state.quantity = 3; // **Lo**gs: "**To**tal: 60"
```

## 6. Real-World **Ex**ample: **Sho**pping **Ca**rt

Let's see how **th**is **wo**rks in a **pra**ctical **ex**ample:

```**jav**ascript
// **Cr**eate a **rea**ctive **sho**pping **ca**rt
**co**nst **ca**rt = **rea**ctive({
  **it**ems: [],
  **to**tal: 0,
});

// **Wa**tch for **ch**anges in the **ca**rt
**wat**chEffect(() => {
  cart.total = cart.items.reduce((sum, **it**em) => sum + item.price, 0);
  console.log(`Cart **to**tal: $${cart.total}`);
});

// Add **it**ems to **ca**rt
cart.items.push({ **na**me: "**Ap**ple", **pr**ice: 1 });
cart.items.push({ **na**me: "**Ba**nana", **pr**ice: 2 });
// **Lo**gs: "**Ca**rt **to**tal: $3"

// **Up**date **it**em **pr**ice
cart.items[0].price = 2;
// **Lo**gs: "**Ca**rt **to**tal: $4"
```

## 7. **Co**mmon **Pat**terns in **Mo**dern **Fra**meworks

### Vue.js **St**yle

```**jav**ascript
**co**nst **st**ate = **rea**ctive({
  **co**unt: 0,
});

**wat**chEffect(() => {
  document.getElementById("counter").textContent = state.count;
});

// **Up**date **wi**ll **aut**omatically **re**flect in the DOM
state.count++;
```

### **Re**act **St**yle

```**jav**ascript
let **st**ate = {
  **co**unt: 0,
};

**fun**ction setState(newState) {
  **st**ate = { ...**st**ate, ...**new**State };
  **re**nder();
}

**fun**ction **re**nder() {
  document.getElementById("counter").textContent = state.count;
}

// **Up**date **wi**ll **tr**igger a re-render
**set**State({ **co**unt: state.count + 1 });
```

## Key **Con**cepts to **Rem**ember

1. ****Rea**ctivity** **me**ans **aut**omatically **upd**ating **wh**en **da**ta **ch**anges
2. ****Dep**endencies** are **tr**acked **wh**en **da**ta is **acc**essed
3. ****Ef**fects** are **fun**ctions **th**at run **wh**en **dep**endencies **ch**ange
4. ****Pr**oxies** and **Getters/Setters** are **to**ols to **ma**ke **ob**jects **rea**ctive
5. ****St**ate **Man**agement** is **ab**out **ke**eping **tr**ack of **ch**anges

## **Co**mmon **Pit**falls

1. ****Cir**cular **Dep**endencies**

```**jav**ascript
// **Av**oid **th**is!
**wat**chEffect(() => {
  state.a = state.b + 1;
});

**wat**chEffect(() => {
  state.b = state.a + 1;
});
```

2. ****Me**mory **Le**aks**

```**jav**ascript
// **Al**ways **cl**ean up **ef**fects **wh**en **do**ne
**co**nst **st**op = **wat**chEffect(() => {
  // **ef**fect **co**de
});

// **Wh**en **com**ponent **unm**ounts
**st**op();
```
