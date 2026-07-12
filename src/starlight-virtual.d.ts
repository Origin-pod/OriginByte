declare module 'virtual:starlight/components/EditLink' {
  const EditLink: (props: Record<string, unknown>) => unknown;
  export default EditLink;
}

declare module 'virtual:starlight/components/LastUpdated' {
  const LastUpdated: (props: Record<string, unknown>) => unknown;
  export default LastUpdated;
}

declare module 'virtual:starlight/components/Pagination' {
  const Pagination: (props: Record<string, unknown>) => unknown;
  export default Pagination;
}

declare module 'virtual:starlight/user-config' {
  const config: {
    credits?: boolean;
    [key: string]: unknown;
  };
  export default config;
}
