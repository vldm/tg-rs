initSidebarItems({"struct":[["LeakyBucket","Implements the industry-standard leaky bucket rate-limiting as-a-meter. The bucket keeps a \"fill height\", pretending to drip steadily (which reduces the fill height), and increases the fill height with every cell that is found conforming. If cells would make the bucket overflow, they count as non-conforming."],["State","Represents the state of a single history of decisions."],["TooEarly","Returned in case of a negative rate-limiting decision."]]});