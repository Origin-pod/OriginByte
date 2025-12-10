import type {SidebarsConfig} from '@docusaurus/plugin-content-docs';

// This runs in Node.js - Don't use client-side code here (browser APIs, JSX...)
// To hide any of the documents add this on top of the file
//---
//sidebar_position: null
//---
/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */
const sidebars: SidebarsConfig = {
  // Main documentation sidebar
  tutorialSidebar: [
    'intro',

    // C++ Section (files not found - commented out for now)
    // {
    //   type: 'category',
    //   label: 'C++',
    //   items: [
    //     'C++/01-C++ Basic Concepts',
    //     'C++/02-C++ Idiomatic Patterns',
    //     'C++/03-Design Patterns',
    //     {
    //       type: 'category',
    //       label: 'Data Structures & Algorithms',
    //       items: [
    //         {
    //           type: 'category',
    //           label: 'Fundamental Data Structures',
    //           items: [
    //             'data structures and algorithms/01-Fundamental Data Structures/01-Arrays',
    //             'data structures and algorithms/01-Fundamental Data Structures/02-Linked Lists',
    //             'data structures and algorithms/01-Fundamental Data Structures/03-Stacks',
    //             'data structures and algorithms/01-Fundamental Data Structures/04-Queues',
    //             'data structures and algorithms/01-Fundamental Data Structures/05-Hash Tables',
    //           ],
    //         },
    //         {
    //           type: 'category',
    //           label: 'Sorting Algorithms',
    //           items: [
    //             'data structures and algorithms/02-Sorting Algorithms/01-Bubble Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/02-Selection Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/03-Insertion Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/04-Merge Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/05-Quick Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/06-Heap Sort',
    //             'data structures and algorithms/02-Sorting Algorithms/07-Counting Sort',
    //           ],
    //         },
    //         {
    //           type: 'category',
    //           label: 'Searching Algorithms',
    //           items: [
    //             'data structures and algorithms/03-Searching Algorithms/01-Linear Search',
    //             'data structures and algorithms/03-Searching Algorithms/02-Binary Search',
    //             'data structures and algorithms/03-Searching Algorithms/03-Jump Search',
    //             'data structures and algorithms/03-Searching Algorithms/04-Interpolation Search',
    //           ],
    //         },
    //         'data structures and algorithms/README',
    //       ],
    //     },
    //   ],
    // },

    // Rust Section (files not found - commented out for now)
    // {
    //   type: 'category',
    //   label: 'Rust',
    //   items: [
    //     'Rust/01-Rust Basic Concepts',
    //     'Rust/02-Rust Idiomatic Patterns',
    //     'Rust/03-Rust Design Patterns',
    //     {
    //       type: 'category',
    //       label: 'Data Structures & Algorithms',
    //       items: [
    //         {
    //           type: 'category',
    //           label: 'Fundamental Data Structures',
    //           items: [
    //             'Rust/data structures and algorithms/01-Fundamental Data Structures/01-Arrays',
    //             'Rust/data structures and algorithms/01-Fundamental Data Structures/02-Linked Lists',
    //             'Rust/data structures and algorithms/01-Fundamental Data Structures/03-Stacks',
    //             'Rust/data structures and algorithms/01-Fundamental Data Structures/04-Queues',
    //             'Rust/data structures and algorithms/01-Fundamental Data Structures/05-Hash Tables',
    //           ],
    //         },
    //         {
    //           type: 'category',
    //           label: 'Sorting Algorithms',
    //           items: [
    //             'Rust/data structures and algorithms/02-Sorting Algorithms/01-Bubble Sort',
    //             'Rust/data structures and algorithms/02-Sorting Algorithms/02-Selection Sort',
    //           ],
    //         },
    //       ],
    //     },
    //   ],
    // },

    // Frontend Development (Deep Dive)
    {
      type: 'category',
      label: 'Frontend Development',
      items: [
        'Frontend/Reactivity',
        'Frontend/DOM Manipulation Architectures',
        'Frontend/Idiomatic_Patterns',
        'Frontend/Decoding Web Performance',
        'Thinking Patterns/Idiomatic_Patterns',
      ],
    },

    // Backend Development
    {
      type: 'category',
      label: 'Backend Development',
      items: [
        'Backend/REST_and_API_Design',
        'Backend/Go_idiomatic_patterns',
      ],
    },

    // System Programming
    {
      type: 'category',
      label: 'System Programming',
      items: [
        'System Programming/idiomatic_patterns',
        'System Programming/Iggy',
      ],
    },

    // BrAIn Section
    {
      type: 'category',
      label: 'BrAIn Project',
      items: [
        'BrAIn/01PRD',
        'BrAIn/01MVPPrd',
        'BrAIn/03TechnicalBase',
        'BrAIn/04TechnicalSpecification',
        'BrAIn/05TechnicalSpecification',
        'BrAIn/06VoiceProcessingComponent',
        'BrAIn/07DatabaseSchema',
        'BrAIn/LaunchChecklist',
      ],
    },

    
    // Random Explorations
    {
      type: 'category',
      label: 'Random Explorations',
      items: [
        'Random Explorations/Why-rails',
        {
          type: 'category',
          label: 'Browser',
          items: [
            'Random Explorations/Browser/BrowserInternals',
            'Random Explorations/Browser/Rendering_Engine',
            'Random Explorations/Browser/Rust-rendering-engine-and-framework',
            'Random Explorations/Browser/RustUI',
          ],
        },
        {
          type: 'category',
          label: 'Databases',
          items: [
            'Random Explorations/Databases/basics',
            'Random Explorations/Databases/database-diagrams',
            'Random Explorations/Databases/practical-examples',
            'Random Explorations/Databases/VectorDB',
          ],
        },
        {
          type: 'category',
          label: 'OS',
          items: [
            'Random Explorations/OS/Kernel',
          ],
        },
        {
          type: 'category',
          label: 'UBI',
          items: [
            'Random Explorations/UBI/UBI',
            'Random Explorations/UBI/Rails_inspiration',
            'Random Explorations/UBI/Usecases',
          ],
        },
      ],
    },

    // Thinking Patterns
    {
      type: 'category',
      label: 'Thinking Patterns',
      items: [
        'Thinking Patterns/Segmentation-Differentiation',
        'Thinking Patterns/Software_Philosophies',
        'Thinking Patterns/Idiomatic_Patterns',
      ],
    },

    // Learning Resources
    {
      type: 'category',
      label: 'Learning Resources',
      items: [
        {
          type: 'category',
          label: 'Book Notes',
          items: ['Book Notes/Residues by Brain O\'Reilly'],
        },
        {
          type: 'category',
          label: 'To-Read Resources',
          items: [
            'To-Read Resources/May-2025',
            'To-Read Resources/Oct-2025',
            'To-Read Resources/Nov-2025',
            'To-Read Resources/opensoource',
            'To-Read Resources/waitlisted',
          ],
        },
        {
          type: 'category',
          label: 'Courses',
          items: ['Courses/Neural-Network-for-Machine-Learning/Neural-Network-for-Machine-Learning-v1.1'],
        },
      ],
    },

    // Conference Talks
    {
      type: 'category',
      label: 'Conference Talks',
      items: [
        'Conf Talks Notes/an_introduction_to_residuality_theory',
        'Conf Talks Notes/RL-Kernels-Reasoning-Quantization-and-Agents-Daniel-Han',
        'Conf Talks Notes/From-Text-to-Vision-to-Voice-Exploring-Multimodality-with-Open-AI-Romain-Huet',
        'Talk-Planning/Talk-topics',
      ],
    },

    // Research Papers
    {
      type: 'category',
      label: 'Research Papers',
      items: [
        'Research Paper/Modeling_Code__Is_Text_All_You_Need',
        'Research Paper/CodeGRAG__Extracting_Composed_Syntax_Graphs',
        'Research Paper/Talk_like_a_graph__Encoding_graphs_for_LLMs',
        'Research Paper/CodeGraph__Enhancing_Graph_Reasoning_of_LLMs_with_Code',
      ],
    },

    // Projects & Tools
    {
      type: 'category',
      label: 'Projects & Tools',
      items: [
        {
          type: 'category',
          label: 'Project Ideas',
          items: [
            'Project Idea Dump/IdeaList',
            'Project Idea Dump/hopHub',
            'Project Idea Dump/Devfolio',
          ],
        },
        {
          type: 'category',
          label: 'LuminaJS',
          items: [
            'Project Idea Dump/LuminaJS/v0.0.0',
            'Project Idea Dump/LuminaJS/LuminaJS',
          ],
        },
        {
          type: 'category',
          label: 'Prompt Library',
          items: [
            'Prompt Library/1000IQ',
            'Prompt Library/prd',
          ],
        },
        {
          type: 'category',
          label: 'HandsFree',
          items: ['Project Idea Dump/HandsFree/InitialPrd'],
        },
        {
          type: 'category',
          label: 'Hacks',
          items: ['Hacks/workflow', 'Hacks/markdown-cheatsheet'],
        },
      ],
    },

    // Theoretical Physics
    {
      type: 'category',
      label: 'Theoretical Physics',
      items: [
        'Theoritical Physics/Definitions',
        'Theoritical Physics/Mind_map',
        'Theoritical Physics/Paper_1',
      ],
    },

    // Additional sections
    {
      type: 'category',
      label: 'Productivity',
      items: ['Productivity/v1'],
    },
    {
      type: 'category',
      label: 'Iggy',
      items: ['Iggy/Intro'],
    },

    // Daily Logs
    {
      type: 'category',
      label: 'Daily Logs',
      items: ['Read of the Day/dailylog'],
    },

    // Templates section removed - _note_template not found in docs
  ],
  
  // Hacks are now a React component page, not a doc page
};

export default sidebars;
