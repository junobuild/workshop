import PropTypes from "prop-types";

export const Backdrop = ({ spinner = false }) => (
  <div className="dark:bg-lavender-blue-200/40 fixed inset-0 z-40 flex items-center justify-center bg-white/30 backdrop-blur-xl">
    {spinner && (
      <div className="border-lavender-blue-600 h-12 w-12 animate-spin rounded-full border-[3px] border-solid border-t-transparent"></div>
    )}
  </div>
);

Backdrop.propTypes = {
  spinner: PropTypes.bool,
};
