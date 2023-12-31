
import Main
-- | 
-- >>> dup [(Int 1)]
-- [1,1]
--
-- >>> dup [(Bool True)]
-- [True,True]
--
-- >>> dup []
-- []
--

-- |
-- >>> swap []
-- []
--
-- >>> swap [(Int 1)]
-- [1]
--
-- >>> swap [(Bool True),(Bool False)]
-- [False,True]
--

-- |
-- >>> pop []
-- []
--
-- >>> pop [(Int 1)]
-- []
--
-- >>> pop [(Bool True),(Bool False)]
-- [False]
--

-- |
-- >>> push (Int 1) []
-- [1]
--
-- >>> push (Bool True) [(Int 1)]
-- [True,1]
--
-- >>> push (List [(Int 1),(Float 2.2222233333)]) [(Bool True),(Bool False)]
-- [[1,2.2222233],True,False]
--

-- |
-- >>> parseInteger []
-- Right []
--
-- >>> parseInteger [(String "2")]
-- Right [2]
--
-- >>> parseInteger [(String "abc")]
-- Left "Error: Unable to parse string as an integer."
--
-- >>> parseInteger [(Bool True)]
-- Right [True]
--

-- |
-- >>> parseFloat []
-- Right []
--
-- >>> parseFloat [(String "2.2")]
-- Right [2.2]
--
-- >>> parseFloat [(String "abc")]
-- Left "Error: Unable to parse string as a float."
--
-- >>> parseFloat [(Bool True)]
-- Right [True]
--

-- |
-- >>> add []
-- Left "Error: Unable to perform addition."
--
-- >>> add [(String "2.2")]
-- Left "Error: Unable to perform addition."
--
-- >>> add [(Int 2),(Int 2)]
-- Right [4]
--
-- >>> add [(Float 2.2),(Float 2.3)]
-- Right [4.5]
--
-- >>> add [(Float 2.2),(Int 2)]
-- Left "Error: Unable to perform addition."
--
-- >>> add [(Int 2),(Float 2.3)]
-- Left "Error: Unable to perform addition."
--

-- |
-- >>> sub []
-- Left "Error: Unable to perform subtraction."
--
-- >>> sub [(String "2.2")]
-- Left "Error: Unable to perform subtraction."
--
-- >>> sub [(Float 2.0),(Float 2.2)]
-- Right [-0.20000005]
--
-- >>> sub [(Float 2.3),(Float 10.5)]
-- Right [-8.2]
--
-- >>> sub [(Float 2.2),(Int 2)]
-- Left "Error: Unable to perform subtraction."
--
-- >>> sub [(Int 2),(Float 2.3)]
-- Left "Error: Unable to perform subtraction."
--

-- |
-- >>> mul []
-- Left "Error: Unable to perform multiplication."
--
-- >>> mul [(String "2.2")]
-- Left "Error: Unable to perform multiplication."
--
-- >>> mul [(Int 2)]
-- Left "Error: Unable to perform multiplication."
--
-- >>> mul [(Float 2.0),(Float 2.2)]
-- Right [4.4]
--
-- >>> mul [(Int 2),(Int 2)]
-- Right [4]
--
-- >>> mul [(Float 2.2),(Int 2)]
-- Left "Error: Unable to perform multiplication."
--
-- >>> mul [(Int 2),(Float 2.3)]
-- Left "Error: Unable to perform multiplication."
--

-- |
-- >>> fdivision []
-- Left "Error: Unable to perform float division."
--
-- >>> fdivision [(String "2.2")]
-- Left "Error: Unable to perform float division."
--
-- >>> fdivision [(Int 2)]
-- Left "Error: Unable to perform float division."
--
-- >>> fdivision [(Float 10.0),(Float 3.0)]
-- Right [3.3333333]
--
-- >>> fdivision [(Int 4),(Int 2)]
-- Right [2.0]
--
-- >>> fdivision [(Float 2.2),(Int 2)]
-- Left "Error: Unable to perform float division."
--
-- >>> fdivision [(Int 2),(Float 2.3)]
-- Left "Error: Unable to perform float division."
--

-- |
-- >>> integerDivision [(Int 4),(Int 2)]
-- Right [2]
--
-- >>> integerDivision [(Int 10),(Int 3)]
-- Right [3]
--
-- >>> integerDivision [(Float 2.2)]
-- Left "Error: Unable to perform integer division."
--
-- >>> integerDivision []
-- Left "Error: Unable to perform integer division."
--

-- |
-- >>> processCommand "?s" [(Int 4),(Int 2)]
-- Current stack: [4,2]
-- [4,2]
--
-- >>> processCommand "+" [(Int 4),(Int 2)]
-- [6]
--
-- >>> processCommand "parseInteger" [(Int 4),(Int 2)]
-- [4,2]
--
-- >>> processCommand "-" [(Float 4.1),(Int 2)]
-- Error: Unable to perform subtraction.
-- [4.1,2]
--
-- >>> processCommand "2 2 +" []
-- Unrecognized command: 2 2 +
-- []
--
-- >>> processCommand "{ 2 2 +}" []
-- [{ 2 2 +}]
--
-- >>> processCommand "{ 2 2 +} 2 2 +" []
-- Unrecognized command: { 2 2 +} 2 2 +
-- []
--
-- >>> processCommand "\"Hello world\"" []
-- ["Hello world"]
--
-- >>> processCommand "\"Hello world\" 2" []
-- Unrecognized command: "Hello world" 2
-- []
--
-- >>> processCommand "push 2" []
-- [2]
--
-- >>> processCommand "push 2 2" []
-- [2]
--
-- >>> processCommand "True" []
-- [True]
--
-- >>> processCommand "False" []
-- [False]
--
-- >>> processCommand "2" []
-- Unrecognized command: 2
--
