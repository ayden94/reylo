// main.tsx
// @ts-ignore
import t from "tropical";

type Route<T, P> = {
  parent?: P;
  load?: () => T;
  render?(props: T): any;
  layout?(props: T): any;
};

type T = {
  parent<P>(): P;
  route<Data = null, Parent = null>(arg: Route<Data, Parent>): Route<Data, Parent>;
};

declare const t: T;

const parent = t.route({
  layout() {
    return "adsf";
  },
  render() {
    return "asdf";
  },
});

const child = t.route({
  parent: t.parent<typeof parent>(),
  load() {
    return { sup: "hello" };
  },
  render(props) {
    return props.sup;
  },
});
child.parent.

export default child;
