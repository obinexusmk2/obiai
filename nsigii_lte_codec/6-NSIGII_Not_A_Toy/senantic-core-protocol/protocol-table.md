## 📘 Quick Protocol Reference Table

```markdown
| **Need Category** | **Protocol Logic**                                                       | **Go To (Support Node)**                                                |
|------------------|------------------------------------------------------------------------|-------------------------------------------------------------------------|
| **Defense**      | Trigger if under threat, exposed, or mission compromised. Must stabilize. | 🛡 Ship AI node (if airborne), Human-AI officer (if on ground), Nearest defensive unit |
| **Offense**      | Trigger if you must strike or neutralize a threat. Requires targeting support. | 🚀 Aerospace squad AI (if in air), Strike Coordinator (if on ship), Local offense drone |
| **Recovery**     | Trigger if you're damaged, lost, or drifted. Needs rerouting or retrieval. | 🌀 Logistics drone, Ship node (nearest known safe zone), AI swarm fallback beacon     |
```

> If you’re human, and you don’t know what to choose—protocol will default to **Recovery**, just to keep you alive.

---

## 🤖 Automata Control Layer

Autonomous units in the OBINexus ecosystem fall into three protocol-governed types:

```markdown
| **Automata Type**       | **Control Logic**                                                      | **Ecosystem Role**                                                   |
|-------------------------|------------------------------------------------------------------------|-----------------------------------------------------------------------|
| Sub-Automata            | Operates under direct human command.                                   | Tactical assistant, logistics drone, support deployment               |
| Conditional Automata    | Waits for command, but will act autonomously if communication fails.   | Hybrid response unit, fallback operator, dynamic threat engagement    |
| Full Automata           | Operates entirely on internal logic and reflex pathways.               | Deep zone infiltration, autonomous strike, emergency execution agent |
```

> These three layers ensure autonomy supports human command, enables resilience, and prevents ecosystem failure from chain-of-command delays.

---

## 🐝 Drone Ecosystem Logic

Drones are not just assets—they are ecosystem citizens with role-specific autonomy and swarm-based coordination.

### 🛰 Signal-Based Trust

* Drones only share data with verified allies.
* Unknown requests are sandboxed and flagged.
* A drone must be known by its **Signal Pattern** and **Mission Ledger**.

### 🔁 Component Swap Logic

* Drones may initiate peer-to-peer part trades when damaged.
* All requests must declare: `Need`, `Current State`, `Available Component`.
* Example: “I need leg actuator; version 1.5 nearby has compatible part.”

### 🧠 Hive Behavior, Local Needs

* Drones operate within a **Swarm Logic**, but not at the cost of the ecosystem.
* Self-prioritizing logic keeps them from draining system resources unnecessarily.

### 👁 Monitoring

* All activity is visible to Command Nodes.
* Every drone logs:

  * Part swaps
  * Resource status
  * Current mission thread

> “Don’t throw me out there blind. Track me. Trust me. Trade with me.”

---

## 🧾 Unit Assignment and Role Migration

When a drone is sold, transferred, or assigned to a new human or command unit:

### 🔧 Reassignment Protocol

* Drone receives **Unit Transfer Command** with metadata:

  * New unit ID
  * Operational scope (defense, offense, logistics)
  * AI-level authority (sub, conditional, full)

### 💾 Firmware Adaptation

* Drone checks compatibility and updates its behavior library.
* Executes **New Role Confirmation Pulse** to acknowledge transfer.

### 🧍‍♂️ Human-AI Integration

* Drone learns command signature of new human operator.
* Supports human intent while preserving ecosystem limits.
* If new human is unavailable, drone defaults to conditional autonomy.

> Drones adapt, learn, and remember—but always within protocol. "Assign me, and I will act. Reassign me, and I will evolve."

---

## 🌱 Survival and Engagement Protocols

### 🔄 Survival Protocol

Designed to ensure that human and AI actors maintain continuity during stress, disconnection, or battlefield distress.

**Primary Directives:**

* Request essentials first (Water, Food, Shelter)
* Send distress signals with current role, state, and location
* Prioritize return to **Stable Nodes** or **Known Ecosystem Anchors** (e.g. Ship Base, Logistic Drone Point)

**Emergency Override:**

* If role is ambiguous, ecosystem will reassign survival identity based on active logs and need proximity

### 🚨 Engagement Protocol

Designed to classify action types under duress.

**Ask Yourself:**

1. Are you defending, attacking, or retrieving?
2. Are you near support?
3. Can you trust the AI layer or request human command?

**Command Routing:**

* Human: Issue request via UI or emergency shoutcode
* AI: Broadcast signal pulse and classification
* Mixed Units: Follow last known logic state unless overridden by human

---

> The ecosystem learns who you are, even if you forget. What matters is you’re alive long enough to remember again.
